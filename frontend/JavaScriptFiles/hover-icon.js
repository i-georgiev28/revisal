const icons = document.querySelectorAll('i.bx');

icons.forEach(icon => {
    const bxClass = Array.from(icon.classList).find(className => className.startsWith('bx-') && className !== 'bx');

    if (bxClass) {
        icon.addEventListener('mouseenter', () => {

            icon.classList.add('bx-tada');
        });

        icon.addEventListener('mouseleave', () => {
            icon.classList.remove('bx-tada');
        });
    }
});