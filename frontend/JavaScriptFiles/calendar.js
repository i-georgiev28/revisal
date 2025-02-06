
    const monthNames = [
      "Януари", 
      "Февруари", 
      "Март", 
      "Април", 
      "Май", 
      "Юни", 
      "Юли", 
      "Август", 
      "Септрември", 
      "Октомври", 
      "Ноември", 
      "Декември"
    ];
    let currentDate = new Date();
    let currentMonth = currentDate.getMonth();
    let currentYear = currentDate.getFullYear();

    function generateCalendar(month, year) {
        // Get the first day of the month and adjust it so Monday is 0 and Sunday is 6
        const firstDay = (new Date(year, month, 1).getDay() + 6) % 7;
        const lastDate = new Date(year, month + 1, 0).getDate();
        const calendarDays = document.getElementById("calendarDays");
        const monthYear = document.getElementById("monthYear");

        calendarDays.innerHTML = "";
        monthYear.textContent = `${monthNames[month]} ${year}`;

        // Generate empty days for the first row if the month doesn't start on Monday
        for (let i = 0; i < firstDay; i++) {
            const emptyDay = document.createElement("div");
            emptyDay.classList.add("day", "disabled");
            calendarDays.appendChild(emptyDay);
        }

        for (let day = 1; day <= lastDate; day++) {
          const dayElement = document.createElement("div");
          dayElement.classList.add("day");
      
          // Calculate the day of the week for this specific day (adjusting for Monday as the first day)
          const dayOfWeek = (firstDay + day - 1) % 7;
      
          // Check if the day is Saturday (5) or Sunday (6)
          if (dayOfWeek === 5 || dayOfWeek === 6) {
              dayElement.classList.add("weekend"); // Add the weekend class
          }
      
          dayElement.textContent = day;
          calendarDays.appendChild(dayElement);
      }

    }

    function changeMonth(increment) {
        currentMonth += increment;
        if (currentMonth > 11) {
            currentMonth = 0;
            currentYear++;
        } else if (currentMonth < 0) {
            currentMonth = 11;
            currentYear--;
        }
        generateCalendar(currentMonth, currentYear);
    }

    // Call generateCalendar() right after the page loads to display the current month
    document.addEventListener("DOMContentLoaded", function() {
        generateCalendar(currentMonth, currentYear);
    });