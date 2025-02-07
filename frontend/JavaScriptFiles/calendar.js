const monthNames = [
    "Януари", "Февруари", "Март", "Април", "Май", "Юни",
    "Юли", "Август", "Септември", "Октомври", "Ноември", "Декември"
];

const daysInMonth = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

let currentDate = new Date();
let currentMonth = currentDate.getMonth();
let currentYear = currentDate.getFullYear();
let selectedDay = null;

function updateDayStyles(dayElement, year, month, day) {
    // If the day is selected or is the current day, make the text white
    if (dayElement.classList.contains('selected-day') || dayElement.classList.contains('current-day')) {
        dayElement.style.color = "white";
    } else {
        // If the day is a weekend (Saturday or Sunday), color it blue
        const dayOfWeek = new Date(year, month, day).getDay();
        if (dayOfWeek === 6 || dayOfWeek === 0) {
            dayElement.style.color = "#4070f4";
            dayElement.style.fontWeight = "500";
        } else {
            dayElement.style.color = "";  // Reset color for weekdays that are not selected
        }
    }
}

function generateCalendar(month, year) {
    const firstDayOfMonth = new Date(year, month, 1).getDay();
    let numDaysInMonth = daysInMonth[month];

    // Handle leap year for February
    if (month === 1 && year % 4 === 0 && (year % 100 !== 0 || year % 400 === 0)) {
        numDaysInMonth = 29;
    }

    const calendarDaysElement = document.getElementById("calendar-days");
    calendarDaysElement.innerHTML = ''; // Clear the previous calendar

    // Adjust the first day so that Monday is 0, Sunday is 6
    const adjustedFirstDay = (firstDayOfMonth === 0) ? 6 : firstDayOfMonth - 1;

    let daysDisplayed = 0;

    // Fill the calendar with empty cells at the start
    for (let i = 0; i < adjustedFirstDay; i++) {
        const emptyCell = document.createElement("div");
        emptyCell.classList.add("day");
        calendarDaysElement.appendChild(emptyCell);
        daysDisplayed++;
    }

    for (let day = 1; day <= numDaysInMonth; day++) {
        const dayElement = document.createElement("div");
        dayElement.classList.add("day");
        dayElement.textContent = day;

        // Highlight "today" and set it as current day
        if (day === currentDate.getDate() && month === currentMonth && year === currentYear) {
            dayElement.classList.add("current-day");
        }

        // Update day styles based on whether it is selected or a weekend
        updateDayStyles(dayElement, year, month, day);

        // Add click event for day selection
        dayElement.addEventListener("click", () => {
            if (selectedDay) {
                selectedDay.classList.remove("selected-day");
                updateDayStyles(selectedDay, currentYear, currentMonth, selectedDay.textContent); // Reset the old day's styles
            }
            dayElement.classList.add("selected-day");
            selectedDay = dayElement;

            // Toggle the "selected-day-window" visibility
            const selectedDaySection = document.querySelector(".selected-day-section");
            if (selectedDaySection) {
                selectedDaySection.style.display = selectedDaySection.style.display === 'none' ? 'block' : 'none';
            }

            //Update
            updateDayStyles(dayElement, year, month, day);
        });

        calendarDaysElement.appendChild(dayElement);
        daysDisplayed++;
    }

    // Fill remaining spots (if necessary) to make exactly 35 days
    while (daysDisplayed < 35) {
        const emptyCell = document.createElement("div");
        emptyCell.classList.add("day");
        calendarDaysElement.appendChild(emptyCell);
        daysDisplayed++;
    }
}

function updateMonthYearDisplay() {
    const monthYearElement = document.getElementById("month-year");
    monthYearElement.textContent = `${monthNames[currentMonth]} ${currentYear}`;
}

document.getElementById("prev-month").addEventListener("click", () => {
    if (currentMonth === 0) {
        currentMonth = 11;
        currentYear--;
    } else {
        currentMonth--;
    }
    generateCalendar(currentMonth, currentYear);
    updateMonthYearDisplay();
});

document.getElementById("next-month").addEventListener("click", () => {
    if (currentMonth === 11) {
        currentMonth = 0;
        currentYear++;
    } else {
        currentMonth++;
    }
    generateCalendar(currentMonth, currentYear);
    updateMonthYearDisplay();
});

generateCalendar(currentMonth, currentYear);
updateMonthYearDisplay();