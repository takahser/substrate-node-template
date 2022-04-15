use frame_support::weights::Weight;

pub trait WeightInfo {
	fn create_profile() -> Weight;
	fn update_profile() -> Weight;
	fn remove_profile() -> Weight;
}

impl WeightInfo for () {
	fn create_profile() -> Weight {
		10_000
	}
	fn update_profile() -> Weight {
		10_000
	}
	fn remove_profile() -> Weight {
		10_000
	}
}
