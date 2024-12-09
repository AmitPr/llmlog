import { $ } from 'bun';

switch (process.argv[2]) {
	case 'clean':
		console.log('Cleaning up db...');
		await $`rm *.sqlite`;
		break;
	case 'setup':
		console.log('Setting up db...');
		await $`bunx drizzle-kit push`;
		break;
	default:
		console.log('Unknown command');
		break;
}
