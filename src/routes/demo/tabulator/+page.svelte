<script lang="ts">
	import { TabulatorWrapper } from '$lib/components/tabulator';
	import type { ColumnDefinition } from 'tabulator-tables';

	// Dummy Data
	let data = [
		{
			id: 1,
			name: 'Billy Bob',
			age: 12,
			gender: 'male',
			height: 1,
			col: 'red',
			dob: '14/09/1984',
			rating: 3
		},
		{
			id: 2,
			name: 'Mary May',
			age: 1,
			gender: 'female',
			height: 2,
			col: 'blue',
			dob: '14/05/1982',
			rating: 4
		},
		{
			id: 3,
			name: 'Christine Lobowski',
			age: 42,
			gender: 'female',
			height: 0,
			col: 'green',
			dob: '22/05/1982',
			rating: 5
		},
		{
			id: 4,
			name: 'Brendon Philips',
			age: 100,
			gender: 'male',
			height: 1,
			col: 'orange',
			dob: '01/08/1980',
			rating: 2
		},
		{
			id: 5,
			name: 'Margret Marmajuke',
			age: 16,
			gender: 'female',
			height: 5,
			col: 'yellow',
			dob: '31/01/1999',
			rating: 1
		}
	];

	// Generate more data
	for (let i = 6; i < 50; i++) {
		data.push({
			id: i,
			name: `User ${i}`,
			age: Math.floor(Math.random() * 100),
			gender: i % 2 === 0 ? 'male' : 'female',
			height: Math.floor(Math.random() * 5),
			col: ['red', 'green', 'blue', 'orange'][Math.floor(Math.random() * 4)],
			dob: '01/01/2000',
			rating: Math.floor(Math.random() * 5) + 1
		});
	}

	let columns: ColumnDefinition[] = [
		{
			title: 'Name',
			field: 'name',
			width: 200,
			editor: 'input',
			headerFilter: 'input',
			frozen: true
		},
		{ title: 'Age', field: 'age', hozAlign: 'left', formatter: 'progress', editor: true },
		{
			title: 'Gender',
			field: 'gender',
			editor: 'list',
			editorParams: { values: ['male', 'female'] },
			width: 100
		},
		{ title: 'Favourite Color', field: 'col', editor: 'input', headerFilter: 'input' },
		{ title: 'Date Of Birth', field: 'dob', hozAlign: 'center', width: 150 },
		{
			title: 'Rating',
			field: 'rating',
			hozAlign: 'center',
			formatter: 'star',
			editor: true,
			width: 120
		}
	];

	let options = {
		height: '100%',
		// Example of "God" options being passed
		rowHeight: 40
	};
</script>

<div class="container mx-auto p-10 min-h-screen">
	<div class="mb-10 text-center">
		<h1
			class="text-4xl font-extrabold mb-4 bg-gradient-to-r from-teal-400 to-blue-600 bg-clip-text text-transparent"
		>
			Tabulator Data Grid
		</h1>
		<p class="text-gray-400">A highly flexible, premium "God Component" for all your data needs.</p>
	</div>

	<div
		class="card p-6 rounded-2xl shadow-2xl bg-gray-900/60 backdrop-blur-xl border border-white/10"
		style="height: 600px;"
	>
		<TabulatorWrapper
			{data}
			{columns}
			{options}
			searchable={true}
			downloadable={true}
			placeholder="Loading Data..."
		/>
	</div>
</div>

<style>
	/* Ensure the page background is dark to match the theme */
	:global(body) {
		background-color: #0f172a; /* Slate 900 */
		color: white;
	}
</style>
