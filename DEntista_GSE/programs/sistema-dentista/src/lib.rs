use anchor_lang::prelude::*; //  LIBRERIA 

// This is your program's public key and it will update
// automatically when you build the project.
declare_id!("7QFCpX7P1Sx2Qo3gYaUvLsWCx3feVaAy4S8GETeNJWdM"); //

#[program]
pub mod sistema_dentista { //BAUTISO PRINCIPAL 
    use super::*;// LIBRERIA 

    // CREATE: Agendar la cita
    pub fn agendar_cita(ctx: Context<AgendarCita>, fecha: i64, servicio: String) -> Result<()> {
        let cita = &mut ctx.accounts.cita;
        let reloj = Clock::get()?; // Obtenemos la hora actual de la red

        // Validación: No agendar en el pasado
        require!(fecha > reloj.unix_timestamp, ErrorDentista::FechaInvalida);

        cita.paciente = *ctx.accounts.paciente.key;
        cita.fecha = fecha;
        cita.servicio = servicio;
        cita.completada = false;
        
        Ok(())
    }

    // UPDATE: Cambiar fecha o marcar como completada
    pub fn reprogramar_cita(ctx: Context<ActualizarCita>, nueva_fecha: i64) -> Result<()> {
        let cita = &mut ctx.accounts.cita;
        cita.fecha = nueva_fecha;
        Ok(())
    }

    // DELETE: Cancelar cita
    pub fn cancelar_cita(_ctx: Context<CancelarCita>) -> Result<()> {
        // Al usar 'close' en el contexto, los SOL regresan al paciente
        Ok(())
    }
}
#[derive(Accounts)]
pub struct AgendarCita<'info> {
    // Definimos el PDA usando la semilla "cita" y la clave del paciente
    #[account(
        init, 
        payer = paciente, 
        space = 8 + 32 + 8 + 4 + 40 + 1,
        seeds = [b"cita", paciente.key().as_ref()], 
        bump
    )]
    pub cita: Account<'info, Cita>,
    #[account(mut)]
    pub paciente: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ActualizarCita<'info> {
    #[account(mut, 
    seeds = [b"cita", paciente.key().as_ref()],
     bump)]
    pub cita: Account<'info, Cita>,
    pub paciente: Signer<'info>,
}

#[derive(Accounts)]
pub struct CancelarCita<'info> {
    #[account(
        mut, 
        seeds = [b"cita", paciente.key().as_ref()], 
        bump,
        close = paciente // Devuelve el dinero del alquiler al paciente
    )]
    pub cita: Account<'info, Cita>,
    pub paciente: Signer<'info>,
}
#[account]
pub struct Cita {
    pub paciente: Pubkey,   // 32
    pub fecha: i64,        // 8 (Unix Timestamp)
    pub servicio: String,   // 4 + 40 (ej: "Limpieza Dental")
    pub completada: bool,   // 1
}

#[error_code]
pub enum ErrorDentista {
    #[msg("La fecha de la cita debe ser en el futuro.")]
    FechaInvalida,
}
