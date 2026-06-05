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
let viewportWidth = window.innerWidth;
let viewportHeight = window.innerHeight;

function updateBallPosition() {
  ball.style.transform = `translate3d(${x}px, ${y}px, 0)`;
}

function animate() {
  const floor = viewportHeight - ballHeight;

  x += horizontalVelocity;
  verticalVelocity += gravity;
  y += verticalVelocity;

  // bounce on floor
  if (y >= floor) {
    y = floor;
    verticalVelocity = -Math.sqrt(2 * gravity * maxFall);
  }

  // left/right wall bounce
  if (x <= 0 || x + ballWidth >= viewportWidth) {
    horizontalVelocity *= -1;
  }

  updateBallPosition();

  requestAnimationFrame(animate);
}

window.addEventListener("resize", function () {
  viewportWidth = window.innerWidth;
  viewportHeight = window.innerHeight;
  x = Math.min(x, viewportWidth - ballWidth);
  y = Math.min(y, viewportHeight - ballHeight);
  updateBallPosition();
});

animate();
