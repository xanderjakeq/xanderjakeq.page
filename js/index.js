const portfolioDiv = document.querySelector('.portfolio')
const workButton = document.querySelector('.expand_button')
const up_arrow = document.getElementById('up_arrow')
const down_arrow = document.getElementById('down_arrow')

workButton.addEventListener('click', (e) => {
    portfolioDiv.classList.toggle('active')
    up_arrow.classList.toggle('hidden')
    down_arrow.classList.toggle('hidden')
})