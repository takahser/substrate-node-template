use frame_support::weights::Weight;

pub trait WeightInfo {
	fn create_vision() -> Weight;
	fn remove_vision() -> Weight;
	fn sign_vision() -> Weight;
	fn unsign_vision() -> Weight;
	fn create_organization() -> Weight;
	fn dissolve_organization() -> Weight;
	fn add_members() -> Weight;
	fn add_tasks() -> Weight;
	fn remove_members() -> Weight;
	fn remove_tasks() -> Weight;
}

impl WeightInfo for () {
	fn create_vision() -> Weight {
		10_000
	}
	fn remove_vision() -> Weight {
		10_000
	}
	fn sign_vision() -> Weight {
		10_000
	}
	fn unsign_vision() -> Weight {
		10_000
	}
	fn create_organization() -> Weight {
		10_000
	}
	fn dissolve_organization() -> Weight {
		10_000
	}
	fn add_members() -> Weight {
		10_000
	}
	fn add_tasks() -> Weight {
		10_000
	}
	fn remove_tasks() -> Weight {
		10_000
	}
	fn remove_members() -> Weight {
		10_000
	}
}
