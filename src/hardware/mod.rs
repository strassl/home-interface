#[derive(PartialEq, Eq, Clone, Copy)]
pub struct Configuration {
    r: u8,
    g: u8,
    b: u8
}

pub trait LightController {
    fn set(&mut self, config: &Configuration);
    fn get(&mut self) -> Configuration;
}

pub struct MockLightController {
    config: Configuration
}

impl LightController for MockLightController {
    fn set(&mut self, config: &Configuration) {
        self.config = config.clone();
    }

    fn get(&mut self) -> Configuration {
        self.config.clone()
    }
}

#[cfg(test)]
mod tests {
    use hardware::LightController;
    use hardware::MockLightController;
    use hardware::Configuration;

    #[test]
    fn get_should_return_value_of_previous_set() {
        let mut controller = MockLightController {
            config: Configuration {
                r: 0,
                g: 0,
                b: 0,
            }
        };

        let new_config = Configuration {
            r: 11,
            g: 12,
            b: 13,
        };
        controller.set(&new_config);

        assert!(controller.get() == new_config);
    }
}
