use crate::domain::repository::register::RegisterRepository;

pub fn start_register(register_repo: &impl RegisterRepository) -> anyhow::Result<()> {
    register_repo.register();
    Ok(())
}

pub fn get_card(register_repo: &impl RegisterRepository) -> anyhow::Result<String> {
    Ok(register_repo.get_card())
}

pub fn is_register(register_repo: &impl RegisterRepository) -> anyhow::Result<bool> {
    Ok(register_repo.is_register_mode())
}
