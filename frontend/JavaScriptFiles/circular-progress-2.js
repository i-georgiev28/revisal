let circularProgress = document.querySelector(".circular-progress"),
progressValue = document.querySelector(".progress-value");
let progressStartValue = 0,    
progressEndValue = 72,    
speed = 20;

let progress = setInterval(() => {
progressStartValue++;
progressValue.textContent = `${progressStartValue}%`
circularProgress.style.background = `conic-gradient(#4070f4 ${progressStartValue * 3.6}deg, #ededed 0deg)`
if(progressStartValue == progressEndValue){
    clearInterval(progress);
}    
}, speed);