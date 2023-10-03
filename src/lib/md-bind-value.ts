export function bindValue(node: any, binding: string) {
	console.log(node);
	node.value = binding;

	const updateValue = () => {
		binding = node.value;
	};

	node.addEventListener('change', updateValue);

	return {
		update(newBinding: string) {
			if (newBinding !== binding) {
				node.value = newBinding;
				binding = newBinding;
			}
		},

		destroy() {
			node.removeEventListener('change', updateValue);
		}
	};
}
