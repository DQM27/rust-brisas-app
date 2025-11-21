<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";

  // --------------------------
  // Empresa
  // --------------------------
  let empresaNombre = "";
  let empresaMsg = "";
  let empresasActivas: any[] = [];

  async function crearEmpresa() {
    empresaMsg = "Creando empresa...";

    try {
      const input = { nombre: empresaNombre };
      const res = await invoke("create_empresa", { input }) as any;

      empresaMsg = `✔ Empresa creada (${res.nombre})`;

      await cargarEmpresas();
      empresaNombre = "";
    } catch (e) {
      empresaMsg = "❌ Error: " + e;
      console.error(e);
    }
  }

  async function cargarEmpresas() {
    try {
      empresasActivas = await invoke("get_empresas_activas") as any[];
    } catch (e) {
      console.error("Error cargando empresas:", e);
    }
  }

  cargarEmpresas();

  // --------------------------
  // Contratista
  // --------------------------
  let cedula = "";
  let nombre = "";
  let apellido = "";
  let fechaVenc = "";
  let empresaSel = "";
  let contratistaMsg = "";

  async function crearContratista() {
    contratistaMsg = "Procesando...";

    try {
      const input = {
        cedula,
        nombre,
        apellido,
        empresaId: empresaSel,
        fechaVencimientoPraind: fechaVenc,

      };

      const res = await invoke("create_contratista", { input }) as any;

      contratistaMsg = "✔ Contratista creado correctamente";
      console.log(res);

      cedula = "";
      nombre = "";
      apellido = "";
      fechaVenc = "";
      empresaSel = "";
    } catch (e) {
      contratistaMsg = "❌ Error: " + e;
      console.error(e);
    }
  }
</script>

<style>
  .panel {
    background: #202020;
    padding: 16px;
    border-radius: 8px;
    margin-bottom: 24px;
  }

  input, select {
    width: 100%;
    padding: 6px;
    margin: 4px 0 10px;
    border-radius: 4px;
    background: #2c2c2c;
    color: white;
    border: 1px solid #444;
  }

  button {
    background: #007acc;
    padding: 8px 14px;
    border-radius: 4px;
    margin-top: 5px;
  }
</style>

<div class="panel">
  <h3>🏢 Crear Empresa</h3>

  <label>Nombre de la empresa:</label>
  <input bind:value={empresaNombre} placeholder="Ej: ACME S.A." />

  <button on:click={crearEmpresa}>Crear Empresa</button>

  <p>{empresaMsg}</p>
</div>

<div class="panel">
  <h3>👷 Registrar Contratista</h3>

  <label>Cédula</label>
  <input bind:value={cedula} />

  <label>Nombre</label>
  <input bind:value={nombre} />

  <label>Apellido</label>
  <input bind:value={apellido} />

  <label>Empresa</label>
  <select bind:value={empresaSel}>
    <option value="">-- Seleccione Empresa --</option>
    {#each empresasActivas as e}
      <option value={e.id}>{e.nombre}</option>
    {/each}
  </select>

  <label>Fecha PRAIND</label>
  <input type="date" bind:value={fechaVenc} />

  <button on:click={crearContratista}>Crear Contratista</button>

  <p>{contratistaMsg}</p>
</div>
