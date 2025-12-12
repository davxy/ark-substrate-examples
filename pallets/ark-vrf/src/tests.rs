use crate::mock::{MaxRingSize, RuntimeOrigin, Test};
use crate::Pallet;
use crate::{mock::new_test_ext, utils};

#[test]
fn build_random_ring() {
    new_test_ext().execute_with(|| {
        let origin = RuntimeOrigin::none();
        let members = utils::ring_members_gen(MaxRingSize::get());
        Pallet::<Test>::push_members(origin.clone(), members, true).unwrap();
        Pallet::<Test>::ring_commit(origin, true).unwrap();
    });
}
