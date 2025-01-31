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

    //grades
    const grades = document.querySelectorAll(".list .grade");

    grades.forEach((grade) => {
        const gradeValue = parseInt(grade.textContent, 10);

        let gradeColor;
        if (gradeValue == 2) 
        {
            gradeColor = "#e74c3c"; //red
        } 
        else if (gradeValue == 3) 
        {
            gradeColor = "#e67e22"; //orange
        } 
        else if (gradeValue == 4){
            gradeColor = "#ffcc00"; //yellow
        }
        else if (gradeValue == 5)
        {
            gradeColor = "#3498db" //blue
        }
        else 
        {
            gradeColor = "#4caf50"; //Green
        }

        grade.style.color = gradeColor;
        grade.style.fontWeight = "bold";
    });
});