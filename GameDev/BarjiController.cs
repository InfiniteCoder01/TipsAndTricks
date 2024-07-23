// 3D character controller from Barji (https://twitch.tv/barji; https://www.youtube.com/@barj)

void FixedUpdate() { // xInput & zInput are wasd controls
	Vector3 direction = new Vector3(xInput, 0, zInput);
	direction = orientation.TransformDirection(direction).normalized;
	direction.y = 0;
	rb.velocity = Accelerate(rb.velocity, direction);
}

Vector3 Accelerate(Vector3 velocity, Vector3 direction) {
	return velocity + (direction * acceleration * Time.fixedDeltaTime) - (velocity * drag);
}
