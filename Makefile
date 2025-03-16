
# Generate new project using cargo generate esp-rs/esp-idf-template cargo
generate-project:
	cargo generate esp-rs/esp-idf-template cargo

post-generate-project:
	sudo apt-get install -y libudev-dev && cargo install espflash ldproxy --locked && espup install

# Build the project
build-water-sensor:
	cd water-sensor && cargo build && espflash flash target/xtensa-esp32-espidf/debug/water-sensor

# Monitor to water-sensor project
monitor-water-sensor:
	cd water-sensor && espflash monitor