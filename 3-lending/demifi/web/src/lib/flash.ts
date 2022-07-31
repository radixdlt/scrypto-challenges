export default function flash(element) {
    requestAnimationFrame(() => {
	element.style.transition = 'none';
	element.style.color = 'rgba(192,192,192,1)';

	setTimeout(() => {
	    element.style.transition = 'color 1s';
	    element.style.color = '';
	});
    });
}
