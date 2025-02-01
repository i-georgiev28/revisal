window.addEventListener('load', function() {
    const menuItems = document.querySelectorAll('.menu li');
    let maxWidth = 0;

    menuItems.forEach(item => {
        const itemWidth = item.offsetWidth;
        if (itemWidth > maxWidth) {
            maxWidth = itemWidth;
        }
    });

    menuItems.forEach(item => {
        item.style.width = `${maxWidth}px`;
    });
});
