// Development helper to test loading states
// Include this script only in development mode

(function () {
  "use strict";

  // Add artificial delay to form submissions for testing
  if (
    window.location.hostname === "localhost" ||
    window.location.hostname === "127.0.0.1"
  ) {
    console.log("🔧 Development mode: Loading state testing enabled");

    // Override form submit to add delay
    const originalSubmit = HTMLFormElement.prototype.submit;
    HTMLFormElement.prototype.submit = function () {
      const form = this;
      console.log("⏱️ Adding 2s delay for loading state testing...");
      setTimeout(() => {
        originalSubmit.call(form);
      }, 2000);
    };

    // Add visual indicator
    const devBadge = document.createElement("div");
    devBadge.style.cssText = `
            position: fixed;
            bottom: 10px;
            left: 10px;
            background: rgba(250, 189, 47, 0.9);
            color: #1d2021;
            padding: 5px 10px;
            border-radius: 5px;
            font-size: 0.75rem;
            font-weight: bold;
            z-index: 10000;
            direction: ltr;
        `;
    devBadge.textContent = "DEV MODE: 2s delay on forms";
    document.body.appendChild(devBadge);
  }
})();
