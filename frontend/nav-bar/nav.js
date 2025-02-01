class NavBar extends HTMLElement {
    connectedCallback() {
        fetch('nav-bar/nav-bar-aside.html')
            .then(response => response.text())
            .then(data => this.innerHTML = data)
            .catch(error => console.error('Error loading nav:', error));
    }
}
customElements.define('nav-bar', NavBar);