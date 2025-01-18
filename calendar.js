const monthNames = 
[
    "Януари",
    "Февруари",
    "Март",
    "Април",
    "Май",
    "Юни",
    "Юли",
    "Август",
    "Септември",
    "Октомври",
    "Ноември",
    "Декември"
];
const daysInMonth = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

let currentDate = new Date();
let currentMonth = currentDate.getMonth();
let currentYear = currentDate.getFullYear();
let selectedDay = null;

function generateCalendar(month, year) {
    const firstDayOfMonth = new Date(year, month, 1).getDay();
    let numDaysInMonth = daysInMonth[month];

    // Handle leap year for February
    if (month === 1 && year % 4 === 0 && (year % 100 !== 0 || year % 400 === 0)) {
        numDaysInMonth = 29;
    }

    const calendarDaysElement = document.getElementById("calendar-days");
    calendarDaysElement.innerHTML = '';

    // Adjust the first day so that Monday is 0, Sunday is 6
    const adjustedFirstDay = (firstDayOfMonth === 0) ? 6 : firstDayOfMonth - 1;

    let daysDisplayed = 0;

    // Fill the calendar with days, ensuring we show exactly 35 days (5 weeks)
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

        // Highlight current day
        if (day === currentDate.getDate() && month === currentMonth && year === currentYear) {
            dayElement.classList.add("current-day");
        }

        // Add click event for day selection
        dayElement.addEventListener("click", () => {
            if (selectedDay) {
                selectedDay.classList.remove("selected-day");
            }
            dayElement.classList.add("selected-day");
            selectedDay = dayElement;
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

// Initial calendar setup
generateCalendar(currentMonth, currentYear);
updateMonthYearDisplay();
