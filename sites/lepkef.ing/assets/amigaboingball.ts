const ball = document.getElementById("amiga-boing-ball");
const ballHeight = ball.offsetHeight;
const ballWidth = ball?.offsetWidth;
const dropHeight = 300;
let x = 0;
let y = window.innerHeight - dropHeight - ballHeight; // start 400px above bottom
let horizontalVelocity = 2;
let verticalVelocity = 0;
const gravity = 0.1;
const targetY = window.innerHeight - ballHeight;
const maxFall = targetY - y;

function animate() {
  const width = window.innerWidth;
  const height = window.innerHeight;
  const floor = height - ballHeight;

  x += horizontalVelocity;
  verticalVelocity += gravity;
  y += verticalVelocity;

  // bounce on floor
  if (y >= floor) {
    y = floor;
    verticalVelocity = -Math.sqrt(2 * gravity * maxFall);
  }

  // left/right wall bounce
  if (x <= 0 || x + ballWidth >= width) {
    horizontalVelocity *= -1;
  }

  ball.style.left = x + "px";
  ball.style.top = y + "px";

  requestAnimationFrame(animate);
}

animate();