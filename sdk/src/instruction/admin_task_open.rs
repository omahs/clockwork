use {
    anchor_client::anchor_lang::{
        solana_program::{
            instruction::{AccountMeta, Instruction},
            pubkey::Pubkey,
            system_program, sysvar,
        },
        InstructionData,
    },
    cronos_program::{
        pda::PDA, 
        state::InstructionData as CronosInstructionData
    }
};

pub fn admin_task_open(
    task_pda: PDA,
    admin: Pubkey,
    authority: Pubkey,
    config: Pubkey,
    daemon: Pubkey,
    ixs: Vec<Instruction>,
    schedule: String,
) -> Instruction {
    Instruction {
        program_id: cronos_program::ID,
        accounts: vec![
            AccountMeta::new(admin, true),
            AccountMeta::new_readonly(authority, false),
            AccountMeta::new_readonly(sysvar::clock::ID, false),
            AccountMeta::new_readonly(config, false),
            AccountMeta::new(daemon, false),
            AccountMeta::new_readonly(system_program::ID, false),
            AccountMeta::new(task_pda.0, false),
        ],
        data: cronos_program::instruction::AdminTaskOpen {
            ixs: ixs.iter().map(|ix| CronosInstructionData::from(ix.clone())).collect(),
            schedule,
            bump: task_pda.1,
        }
        .data(),
    }
}