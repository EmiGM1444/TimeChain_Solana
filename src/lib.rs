use anchor_lang::prelude::*;

// ID del programa
declare_id!("zrHyeBt6Gk26bEqDrSahsmin7hciMTcNX172jJXiz3i");

#[program]
pub mod timechain_solana {
    use super::*;

    // 1. CREATE (PDA): Inicializa la bitácora del freelancer
    pub fn inicializar_tracker(ctx: Context<CrearTracker>, nombre_freelancer: String) -> Result<()> {
        let tracker = &mut ctx.accounts.tracker;
        tracker.owner = ctx.accounts.owner.key();
        tracker.nombre_freelancer = nombre_freelancer;
        tracker.registros = Vec::new();
        
        msg!("Time Tracker creado para el freelancer: {}", tracker.nombre_freelancer);
        Ok(())
    }

    // 2. CREATE (Dato): Registra horas trabajadas en un proyecto (Por defecto no pagado)
    pub fn registrar_horas(ctx: Context<GestionarRegistro>, proyecto: String, horas: u8) -> Result<()> {
        let tracker = &mut ctx.accounts.tracker;
        require!(tracker.owner == ctx.accounts.owner.key(), Errores::NoEresElOwner);

        let nuevo_registro = RegistroSemanal {
            proyecto,
            horas_dedicadas: horas,
            pagado: false, // Por defecto, el trabajo nuevo no está pagado
        };

        tracker.registros.push(nuevo_registro);
        msg!("Horas registradas con éxito.");
        Ok(())
    }

    // 3. UPDATE: Actualiza las horas o cambia el estado a pagado
    pub fn editar_registro(ctx: Context<GestionarRegistro>, proyecto: String, nuevas_horas: u8, esta_pagado: bool) -> Result<()> {
        let tracker = &mut ctx.accounts.tracker;
        require!(tracker.owner == ctx.accounts.owner.key(), Errores::NoEresElOwner);

        let lista = &mut tracker.registros;
        for i in 0..lista.len() {
            if lista[i].proyecto == proyecto {
                lista[i].horas_dedicadas = nuevas_horas;
                lista[i].pagado = esta_pagado;
                
                if esta_pagado {
                    msg!("¡Genial! El proyecto '{}' ha sido marcado como PAGADO.", proyecto);
                } else {
                    msg!("Registro de '{}' actualizado.", proyecto);
                }
                return Ok(());
            }
        }
        Err(Errores::RegistroNoEncontrado.into())
    }

    // 4. DELETE: Elimina un registro (ej. a final de mes para limpiar)
    pub fn eliminar_registro(ctx: Context<GestionarRegistro>, proyecto: String) -> Result<()> {
        let tracker = &mut ctx.accounts.tracker;
        require!(tracker.owner == ctx.accounts.owner.key(), Errores::NoEresElOwner);

        let lista = &mut tracker.registros;
        let index = lista.iter().position(|r| r.proyecto == proyecto);

        if let Some(i) = index {
            lista.remove(i);
            msg!("Registro del proyecto '{}' eliminado.", proyecto);
            Ok(())
        } else {
            Err(Errores::RegistroNoEncontrado.into())
        }
    }

    // 5. READ: Visualiza todos los registros
    pub fn ver_registros(ctx: Context<GestionarRegistro>) -> Result<()> {
        msg!("Freelancer: {}", ctx.accounts.tracker.nombre_freelancer);
        msg!("Bitácora de Horas: {:#?}", ctx.accounts.tracker.registros);
        Ok(())
    }
}

// --- ESTRUCTURAS DE DATOS (ESTADO) ---

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace, PartialEq, Debug)]
pub struct RegistroSemanal {
    #[max_len(30)]
    pub proyecto: String,
    pub horas_dedicadas: u8, // Max 255 horas, ideal para registros semanales/mensuales
    pub pagado: bool,
}

#[account]
#[derive(InitSpace)]
pub struct GestorHoras {
    pub owner: Pubkey,
    #[max_len(40)]
    pub nombre_freelancer: String,
    #[max_len(15)] // Capacidad para gestionar hasta 15 proyectos concurrentes
    pub registros: Vec<RegistroSemanal>,
}

// --- CONTEXTOS ---

#[derive(Accounts)]
pub struct CrearTracker<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        init,
        payer = owner,
        space = 8 + GestorHoras::INIT_SPACE,
        seeds = [b"tracker", owner.key().as_ref()],
        bump
    )]
    pub tracker: Account<'info, GestorHoras>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct GestionarRegistro<'info> {
    pub owner: Signer<'info>,
    #[account(mut)]
    pub tracker: Account<'info, GestorHoras>,
}

// --- ERRORES ---

#[error_code]
pub enum Errores {
    #[msg("Acceso denegado: No eres el dueño de esta bitácora.")]
    NoEresElOwner,
    #[msg("El proyecto buscado no existe en tu bitácora.")]
    RegistroNoEncontrado,
}
