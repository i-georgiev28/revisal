function updateDifficultyColors() {
    const spans = document.querySelectorAll('span.property-difficulty-value');
    
    spans.forEach(span => {
        const ratingText = span.textContent;
        
        const ratingValue = parseInt(ratingText.split('/')[0]);
        
        if (ratingValue <= 3) 
        {
            span.style.color = '#4caf50';
        } 
        else if (ratingValue <= 6) 
        {
            span.style.color = '#ffcc00';
        } 
        else 
        {
            span.style.color = '#ff4d4d';
        }
    });
}


updateDifficultyColors();