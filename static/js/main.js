document.addEventListener("DOMContentLoaded", function () {
  // Auto-dismiss flash messages after 5 seconds
  const flashMessages = document.querySelectorAll(".flash-message");
  flashMessages.forEach(function (message) {
    setTimeout(function () {
      message.style.opacity = "0";
      message.style.transform = "translateY(-20px)";
      setTimeout(function () {
        message.style.display = "none";
      }, 300);
    }, 5000);
  });

  // Handle logout form
  const logoutForm = document.getElementById("logout-form");
  if (logoutForm) {
    const logoutBtn = document.getElementById("logout-btn");

    logoutForm.addEventListener("submit", function (e) {
      // Update button state
      logoutBtn.disabled = true;
      logoutBtn.classList.add("btn-loading-state");
      logoutBtn.querySelector(".btn-content").style.display = "none";
      logoutBtn.querySelector(".btn-loading").style.display = "inline-flex";
    });
  }
});
