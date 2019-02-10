const portfolioDiv = document.querySelector('.portfolio')
const workButton = document.querySelector('.workButton')

workButton.addEventListener('click', (e) => {
    portfolioDiv.classList.toggle('active')
})