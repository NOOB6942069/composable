/// Creates an event with contract’s default prefix and given action attribute.
pub(crate) fn make_event(action: &str) -> cosmwasm_std::Event {
	cosmwasm_std::Event::new(xc_core::gateway::EVENT_PREFIX).add_attribute("action", action)
}
