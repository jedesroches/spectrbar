use battery::{units::ratio::percent, Battery};
use std::{fmt::Display, slice::IterMut};

#[derive(Debug)]
pub struct BatteryStatus {
    pub(crate) charging: bool,
    pub(crate) charges: Vec<f32>,
}

impl Display for BatteryStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // https://github.com/rust-lang/rust/issues/79524
        // Usage of a Vec instead of just normal iterators here is due to the existence
        // of the join(): Vec[T] -> T -> Vec[T] method, whereas intersperse on iterators
        // is still an experimental feature.
        // TODO: use iterators once the above is in stable Rust.
        let mut element_builder: Vec<String> = Vec::new();

        if self.charging {
            element_builder.push("⚇".to_string());
        }

        let mut formatted_charges = self
            .charges
            .iter()
            .map(|c| charge_to_string(*c))
            .collect::<Vec<String>>();

        element_builder.append(&mut formatted_charges);

        write!(f, "{}", element_builder.join(" "))?;
        Ok(())
    }
}

pub fn update_battery(manager: &battery::Manager, batteries: IterMut<Battery>) -> BatteryStatus {
    let mut charging: bool = false;
    let mut charges: Vec<f32> = Vec::new();

    for battery in batteries {
        if let Ok(()) = manager.refresh(battery) {
            charging = charging || battery.state() == battery::State::Charging;
            charges.push(battery.state_of_charge().get::<percent>())
        }
    }
    BatteryStatus { charging, charges }
}

fn charge_to_string(charge: f32) -> String {
    if charge <= 15.0 {
        format!("+@fg=1;{:.0}+@fg=0;", charge)
    } else {
        format!("{:.0}", charge)
    }
}

#[cfg(test)]
mod tests {
    use super::BatteryStatus;

    #[test]
    fn format_shouldbe_empty_when_no_charge_no_batteries() {
        let sut = BatteryStatus {
            charging: false,
            charges: Vec::new(),
        };
        let result = format!("{}", sut);
        assert!(result.is_empty());
    }

    #[test]
    fn format_shouldbe_charging_when_charge_no_batteries() {
        let sut = BatteryStatus {
            charging: true,
            charges: Vec::new(),
        };
        let result = format!("{}", sut);
        assert_eq!("⚇", result);
    }

    #[test]
    fn format_shouldbe_number_when_no_charge_one_battery() {
        let charges = vec![42.0];
        let sut = BatteryStatus {
            charging: false,
            charges,
        };
        let result = format!("{}", sut);
        assert_eq!("42", result);
    }

    #[test]
    fn format_should_round_f32() {
        let charges: Vec<f32> = vec![42.3, 42.7];
        let sut = BatteryStatus {
            charging: false,
            charges,
        };
        let result = format!("{}", sut);
        assert_eq!("42 43", result);
    }

    #[test]
    fn format_shouldbe_charge_then_charges() {
        let charges = vec![70.0, 60.0, 50.0];
        let sut = BatteryStatus {
            charging: true,
            charges,
        };
        let result = format!("{}", sut);
        assert_eq!("⚇ 70 60 50", result);
    }

    #[test]
    fn format_shouldcolor_whenlowbat() {
        let charges = vec![15.0, 16.0];
        let sut = BatteryStatus {
            charging: false,
            charges,
        };
        let result = format!("{}", sut);
        assert_eq!("+@fg=1;15+@fg=0; 16", result);
    }
}
