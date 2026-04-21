# TimeChain Solana - Registro de Horas Freelance ⏱️

**TimeChain Solana** es una dApp (Aplicación Descentralizada) basada en contratos inteligentes que permite a desarrolladores independientes y freelancers registrar sus horas de trabajo directamente en la blockchain de Solana, garantizando transparencia y trazabilidad con sus clientes.

## 🎯 Caso de Uso
Llevar un control en hojas de cálculo tradicionales puede ser alterado o borrado. Al llevar el registro On-Chain mediante este programa:
1. El freelancer tiene un registro inmutable de su tiempo dedicado.
2. Se puede visualizar fácilmente qué proyectos están pendientes de pago y cuáles ya fueron liquidados.
3. Se demuestra dominio en la persistencia de estados y manipulación de memoria con Anchor Framework.

---

## 🛠️ Detalles Técnicos y Estructura

El programa utiliza **Program Derived Addresses (PDAs)** para garantizar que cada freelancer tenga una bitácora única vinculada matemáticamente a su wallet (`[b"tracker", owner.key()]`).

### Estructura de la PDA (`GestorHoras`)
* **`owner`**: (Pubkey) La billetera que desplegó y controla la cuenta.
* **`nombre_freelancer`**: (String) Nombre o alias del profesional.
* **`registros`**: (Vector) Una matriz dinámica que almacena los proyectos. Limitada a 15 elementos para calcular el `InitSpace` y optimizar la renta de Solana.

### Objeto Interno (`RegistroSemanal`)
* **`proyecto`**: (String) Nombre del cliente o proyecto.
* **`horas_dedicadas`**: (u8) Tipo de dato ligero que soporta hasta 255 horas.
* **`pagado`**: (bool) Estado de facturación. Se inicializa en `false` por defecto al crear un registro para proteger la lógica de negocio.

---

## 🔄 Operaciones (CRUD)

| Función | Descripción de la Lógica |
| :--- | :--- |
| `inicializar_tracker` | Crea la PDA y asigna la propiedad de la cuenta al Signer. |
| `registrar_horas` | Añade un proyecto y horas. Infiere automáticamente `pagado = false`. |
| `editar_registro` | Permite sobrescribir las horas (si se trabajó más tiempo) o cambiar la bandera booleana a `true` al recibir el pago del cliente. |
| `eliminar_registro` | Remueve un proyecto concluido del vector iterando mediante funciones nativas de Rust. |
| `ver_registros` | Emite la bitácora completa a los logs On-Chain (`msg!`). |
