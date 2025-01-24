document.addEventListener('DOMContentLoaded', () => {
    const form = document.getElementById('create-pack-form');
    const nextButton = document.getElementById('next');
    const prevButton = document.getElementById('prev');
    const steps = document.querySelectorAll('.step');
    const circles = document.querySelectorAll('.steps .circle');
    const progressBar = document.querySelector('.progress-bar .indicator');
    let currentStep = 0;

    // Function to validate the current step
    const validateStep = () => {
        const currentInputs = steps[currentStep].querySelectorAll('input, select');
        const allFilled = Array.from(currentInputs).every(input => input.value.trim() !== '');

        // Enable "Next" button only if all fields are filled
        nextButton.disabled = !allFilled;
    };

    // Function to show the next step
    const showNextStep = () => {
        if (currentStep < steps.length - 1) {
            steps[currentStep].classList.remove('active');
            circles[currentStep].classList.remove('active');
            currentStep++;
            steps[currentStep].classList.add('active');
            circles[currentStep].classList.add('active');

            // Update progress bar width
            progressBar.style.width = `${((currentStep + 1) / circles.length) * 100}%`;

            // Enable "Prev" button after first step
            prevButton.disabled = currentStep === 0;

            // Validate the new current step
            validateStep();
        }
    };

    // Function to show the previous step
    const showPrevStep = () => {
        if (currentStep > 0) {
            steps[currentStep].classList.remove('active');
            circles[currentStep].classList.remove('active');
            currentStep--;
            steps[currentStep].classList.add('active');
            circles[currentStep].classList.add('active');

            // Update progress bar width
            progressBar.style.width = `${((currentStep + 1) / circles.length) * 100}%`;

            // Enable "Prev" button when not on the first step
            prevButton.disabled = currentStep === 0;

            // Validate the new current step
            validateStep();
        }
    };

    // Event listener for "Next" button
    nextButton.addEventListener('click', (e) => {
        e.preventDefault();  // Prevent form submission
        if (!nextButton.disabled) {
            showNextStep();
        }
    });

    // Event listener for "Previous" button
    prevButton.addEventListener('click', (e) => {
        e.preventDefault();  // Prevent form submission
        showPrevStep();
    });

    // Event listener for any input field to trigger validation
    form.addEventListener('input', validateStep);

    // Initial setup: show first step
    steps.forEach((step, index) => {
        if (index === 0) {
            step.classList.add('active');
            circles[index].classList.add('active');
        } else {
            step.classList.remove('active');
            circles[index].classList.remove('active');
        }
    });

    // Validate on the first step
    validateStep();
});
