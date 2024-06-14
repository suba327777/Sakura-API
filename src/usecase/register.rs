use crate::domain::repository::register::RegisterRepository;

pub fn start_register(register_repo: &impl RegisterRepository) -> anyhow::Result<()> {
    Ok(register_repo.register())
}


pub fn get_card(register_repo: &impl RegisterRepository) -> anyhow::Result<String> {
    Ok(register_repo.get_card())
}