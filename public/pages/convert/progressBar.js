var fillBar = document.getElementById("fill-bar");

function updateProgress(value, max) {
  fillBar.style.width = (value / max) * 100 + "%";
}
