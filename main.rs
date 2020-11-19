use std::collections::HashMap;

#[derive(Eq, PartialEq, Hash)]
enum Health {
  Healthy,
  Incubating,
  Asymptomatic,
  Symptomatic,
  Immune,
}

static ALL_HEALTHS: Vec<Health> = vec![
  Health::Healthy,
  Health::Incubating,
  Health::Asymptomatic,
  Health::Symptomatic,
  Health::Immune,
];

#[derive(Eq, PartialEq, Hash)]
struct WorldState {
  spencer: Health,
  yam: Health,
}
// impl PartialEq for WorldState {}
// impl Eq for WorldState {}

// static mydict: HashMap<> = HashMap::new();

fn prior(h: Health) -> f64 {
  match h {    
    Health::Healthy => 0.98,
    Health::Incubating => 0.01/3.0,
    Health::Asymptomatic => 0.01/3.0,
    Health::Symptomatic => 0.01/3.0,
    Health::Immune => 0.01,
  }
}

const p_transmission: f64 = 0.05;
const p_asymptomatic: f64 = 0.50;

fn interact(me: Health, other: Health) -> (fn(Health) -> f64) {
  match me {
    Health::Healthy => match other {
      Health::Healthy | Health::Incubating | Health::Immune => |h| match h {
        Health::Healthy => 1.0,
        _ => 0.0,
      },
      
      Health::Asymptomatic | Health::Symptomatic => |h| match h {
        Health::Healthy => (1.0 - p_transmission),
        Health::Incubating => p_transmission,
        _ => 0.0,
      },
    },

    Health::Incubating => |h| match h {
      Health::Asymptomatic => p_asymptomatic,
      Health::Symptomatic => 1.0 - p_asymptomatic,
      _ => 0.0,
    },
    Health::Asymptomatic => |h| match h {
      Health::Immune => 1.0,
      _ => 0.0,
    },
    Health::Symptomatic => |h| match h {
      Health::Immune => 1.0,
      _ => 0.0,
    },

    Health::Immune => |h| match h {
      Health::Immune => 1.0,
      _ => 0.0,
    }
  }
}

fn main() {
  let mut day1: &HashMap<WorldState, f64> = &HashMap::new();
  for spencerHealth in ALL_HEALTHS {
    for yamHealth in ALL_HEALTHS {
      day1[&WorldState{spencer: spencerHealth, yam: yamHealth}] = prior(spencerHealth) * prior(yamHealth)
    }
  }

  let mut day2: &HashMap<WorldState, f64> = &HashMap::new();
  for oldSpencerHealth in ALL_HEALTHS {
    for oldYamHealth in ALL_HEALTHS {
      let newSpencerPdf = interact(oldSpencerHealth, oldYamHealth);
      let newYamPdf = interact(oldSpencerHealth, oldYamHealth);
      for newSpencerHealth in ALL_HEALTHS {
        for newYamHealth in ALL_HEALTHS {
          *day2.entry(WorldState{spencer: newSpencerHealth, yam: newYamHealth}).or_insert(0.0) +=
            day1[&WorldState{spencer: oldSpencerHealth, yam: oldYamHealth}]
            * newSpencerPdf(newSpencerHealth)
            * newYamPdf(newYamHealth)
        }
      }
    }
  }


  // // day1 is Healthy, Healthy:
  // *day2.entry({spencer: Health::Healthy, yam: Health::Healthy}).or_insert(0) += day1[{spencer: Health::Healthy, yam: Health::Healthy}]
  // // day1 is Healthy, Incubating:
  // *day2.entry({spencer: Health::Healthy, yam: Health::Asymptomatic}).or_insert(0) += (
  //   day1[{spencer: Health::Healthy, yam: Health::Incubating}]
  //   * p_asymptomatic
  //   * (1 - p_transmission)
  // )
  // *day2.entry({spencer: Health::Healthy, yam: Health::Symptomatic}).or_insert(0) += (
  //   day1[{spencer: Health::Healthy, yam: Health::Incubating}]
  //   * (1 - p_asymptomatic)
  //   * (1 - p_transmission)
  // )
  // *day2.entry({spencer: Health::Incubating, yam: Health::Asymptomatic}).or_insert(0) += (
  //   day1[{spencer: Health::Healthy, yam: Health::Incubating}]
  //   * p_asymptomatic
  //   * p_transmission
  // )
  // *day2.entry({spencer: Health::Incubating, yam: Health::Symptomatic}).or_insert(0) += (
  //   day1[{spencer: Health::Healthy, yam: Health::Incubating}]
  //   * (1 - p_asymptomatic)
  //   * p_transmission
  // )
  // // day1 is Incubating, Healthy:
  // *day2.entry({spencer: Health::Asymptomatic, yam: Health::Healthy}).or_insert(0) += (
  //   day1[{spencer: Health::Incubating, yam: Health::Healthy}]
  //   * p_asymptomatic
  //   * (1 - p_transmission)
  // )
  // *day2.entry({spencer: Health::Symptomatic, yam: Health::Healthy}).or_insert(0) += (
  //   day1[{spencer: Health::Incubating, yam: Health::Healthy}]
  //   * (1 - p_asymptomatic)
  //   * (1 - p_transmission)
  // )
  // *day2.entry({spencer: Health::Asymptomatic, yam: Health::Incubating}).or_insert(0) += (
  //   day1[{spencer: Health::Incubating, yam: Health::Healthy}]
  //   * p_asymptomatic
  //   * p_transmission
  // )
  // *day2.entry({spencer: Health::Symptomatic, yam: Health::Incubating}).or_insert(0) += (
  //   day1[{spencer: Health::Incubating, yam: Health::Healthy}]
  //   * (1 - p_asymptomatic)
  //   * p_transmission
  // )
  // // day1 is Incubating, Incubating:
  // *day2.entry({spencer: Health::Asymptomatic, yam: Health::Asymptomatic}).or_insert(0) += (
  //   day1[{yam: Health::Incubating, spencer: Health::Incubating}]
  //   * p_asymptomatic**2
  // )
  // *day2.entry({spencer: Health::Symptomatic, yam: Health::Symptomatic}).or_insert(0) += (
  //   day1[{yam: Health::Incubating, spencer: Health::Incubating}]
  //   * (1 - p_asymptomatic)**2
  // )
  // *day2.entry({spencer: Health::Symptomatic, yam: Health::Asymptomatic}).or_insert(0) += (
  //   day1[{yam: Health::Incubating, spencer: Health::Incubating}]
  //   * (1 - p_asymptomatic) * p_asymptomatic
  // )
  // *day2.entry({spencer: Health::Asymptomatic, yam: Health::Symptomatic}).or_insert(0) += (
  //   day1[{yam: Health::Incubating, spencer: Health::Incubating}]
  //   * (1 - p_asymptomatic) * p_asymptomatic
  // )
  
  println!("Hello world!");
}
