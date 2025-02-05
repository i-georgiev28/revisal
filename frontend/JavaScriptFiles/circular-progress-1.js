document.querySelectorAll('.circular-progress, .circular-progress-2, .circular-progress-3').forEach((circularProgress, index) => {
    let progressValue = circularProgress.querySelector('.progress-value, .progress-value-2, .progress-value-3');
    let progressStartValue = 0;
    let progressEndValue = [77, 62, 80][index];
    let speed = 20;

    let progress = setInterval(() => {
        progressStartValue++;
        progressValue.textContent = `${progressStartValue}%`;

        circularProgress.style.background = `conic-gradient(#4070f4 ${progressStartValue * 3.6}deg, #ededed 0deg)`;

        if (progressStartValue == progressEndValue) {
            clearInterval(progress);
        }
    }, speed)
});
