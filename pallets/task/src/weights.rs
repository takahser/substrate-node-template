use frame_support::weights::Weight;

pub trait WeightInfo {
	fn create_task() -> Weight;
	fn start_task() -> Weight;
	fn complete_task() -> Weight;
	fn remove_task() -> Weight;
}

impl WeightInfo for () {
	fn create_task() -> Weight {
		10_000
	}
	fn start_task() -> Weight {
		10_000
	}
	fn complete_task() -> Weight {
		10_000
	}
	fn remove_task() -> Weight {
		10_000
	}
}
