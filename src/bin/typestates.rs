// Topic: Typestates
//
// Summary:
//   An airline wants to reduce the amount of lost luggage by
//   ensuring luggage is properly tracked.
//
// Requirements:
// * Implement a luggage tracking system using the typestate pattern
// * Each piece of luggage has a tracking id
// * Luggage goes through multiple states at the airport:
//   * BeginCustody    (luggage is gotten id number)
//   * Check-in        (passenger gives luggage to airport)
//   * OnLoading       (luggage is loaded onto correct plane)
//   * Offloading      (luggage is taken off plane at destination)
//   * AwaitingPickup  (luggage is at destination waiting for passenger pickup)
//   * EndCustody      (luggage was picked up by passenger)
// Notes:
// * Optionally use generics for each state

#[derive(Clone, Copy)]
struct LuggageId(usize);
struct Luggage<State> {
    id: LuggageId,
    state: State,
}

impl<State> Luggage<State> {
    fn next<NextState>(self, state: NextState) -> Luggage<NextState> {
        Luggage { id: self.id, state }
    }
}

struct BeginCustody;
struct CheckIn;
struct OnLoading;
struct OffLoading;
struct AwaitingPickup;
struct EndCustody(LuggageId);

impl Luggage<BeginCustody> {
    fn new(id: LuggageId) -> Self {
        Luggage {
            id,
            state: BeginCustody,
        }
    }

    fn check_in(self) -> Luggage<CheckIn> {
        self.next(CheckIn)
    }
}

impl Luggage<CheckIn> {
    fn onload(self) -> Luggage<OnLoading> {
        self.next(OnLoading)
    }
}

impl Luggage<OnLoading> {
    fn offload(self) -> Luggage<OffLoading> {
        self.next(OffLoading)
    }
}

impl Luggage<OffLoading> {
    fn conveyor(self) -> Luggage<AwaitingPickup> {
        self.next(AwaitingPickup)
    }
}

impl Luggage<AwaitingPickup> {
    fn pickup(self) -> (Luggage<EndCustody>, EndCustody) {
        let id = self.id;
        (self.next(EndCustody(id)), EndCustody(id))
    }
}

fn main() {
    let id = LuggageId(1);
    let luggage = Luggage::new(id);
    let luggage = luggage.check_in().onload().offload().conveyor();
    let (_luggage, _) = luggage.pickup();
}
