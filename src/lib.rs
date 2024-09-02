use concordium_std::*;

#[init(contract = "hello_world")]
fn init(_ctx: &InitContext, _state_builder: &mut StateBuilder) -> InitResult<()> {
    Ok(())
}

#[receive(contract = "hello_world", name = "say_hello")]
fn say_hello(_ctx: &ReceiveContext, _host: &Host) -> ReceiveResult<String> {
    Ok("Hello, World!".to_string())
}