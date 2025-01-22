class FooterBar extends HTMLElement {
    connectedCallback() {
        fetch('footer/footer.html')  // Path to footer.html
            .then(response => response.text())
            .then(data => this.innerHTML = data)
            .catch(error => console.error('Error loading footer:', error));
    }
}
customElements.define('footer-bar', FooterBar);