<script lang="ts">
  export let loading = false;
  export let onSubmit: (data: {
    email: string;
    password: string;
    nombre: string;
    apellido: string;
    role: string;
  }) => void;

  let email = "";
  let password = "";
  let nombre = "";
  let apellido = "";
  let role = "user";

  // Para que el padre pueda hacer reset()
  export function reset() {
    email = "";
    password = "";
    nombre = "";
    apellido = "";
    role = "user";
  }

  function handleSubmit(e: Event) {
    e.preventDefault();
    onSubmit({ email, password, nombre, apellido, role });
  }
</script>

<form on:submit={handleSubmit}>
  <input bind:value={email} placeholder="Email" />
  <input bind:value={password} placeholder="Password" type="password" />
  <input bind:value={nombre} placeholder="Nombre" />
  <input bind:value={apellido} placeholder="Apellido" />
  <select bind:value={role}>
    <option value="user">Usuario</option>
    <option value="admin">Administrador</option>
  </select>

  <button disabled={loading} type="submit">
    {loading ? "Cargando..." : "Registrar"}
  </button>
</form>
