//*skill bars
document.addEventListener("DOMContentLoaded", () => {
    const progressBars = document.querySelectorAll(".skill-bar .skill-per");

    progressBars.forEach((bar) => {
        const tooltip = bar.querySelector(".tooltip");
        let percentage;

        if (tooltip) {
            percentage = parseInt(tooltip.textContent, 10) || 0;
        } else {
            percentage = 0;
            const newTooltip = document.createElement("span");
            newTooltip.className = "tooltip";
            newTooltip.textContent = `${percentage}%`;
            bar.appendChild(newTooltip);
        }

        bar.style.width = `${percentage}%`;
        let barColor;

        if (percentage < 40) {
            barColor = "#ff4d4d"; //Red
        } else if (percentage < 70) {
            barColor = "#ffcc00"; //Yellow
        } else {
            barColor = "#4caf50"; //Green
        }

        bar.style.backgroundColor = barColor;
        if (tooltip) {
            tooltip.style.backgroundColor = barColor;
            tooltip.setAttribute("data-color", barColor);
        }
    });
});