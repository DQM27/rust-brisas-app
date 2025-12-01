// Ejecuta esto en la consola del navegador para corregir el UUID:

// Ver el usuario actual
console.log('Usuario actual:', JSON.parse(localStorage.getItem('brisas-user')));

// Actualizar con el UUID correcto del seed
const user = JSON.parse(localStorage.getItem('brisas-user'));
user.id = '1a90659c-511b-46c3-90e7-878b95ffc02b';
localStorage.setItem('brisas-user', JSON.stringify(user));

console.log('Usuario actualizado:', JSON.parse(localStorage.getItem('brisas-user')));

// Recarga la página
location.reload();
