<script lang="ts">
  import { User, Shield, BarChart3 } from "lucide-svelte";
</script>

<div class="welcome-panel">
  <div class="welcome-content">
    <h1>Bienvenido a Brisas App</h1>
    <p>Sistema de gestión de usuarios</p>

    <div class="features">
      <div class="feature-card">
        <div class="icon">
          <User />
        </div>
        <h3>Gestión de Usuarios</h3>
        <p>Crea, edita y administra usuarios del sistema</p>
      </div>

      <div class="feature-card">
        <div class="icon">
          <Shield />
        </div>
        <h3>Autenticación Segura</h3>
        <p>Login con hashing Argon2 + sesiones persistentes</p>
      </div>

      <div class="feature-card">
        <div class="icon">
          <BarChart3 />
        </div>
        <h3>Panel de Control</h3>
        <p>Métricas y estadísticas en tiempo real</p>
      </div>
    </div>
  </div>
</div>

<style>
  .welcome-panel {
    height: 100%;
    display: flex;
    align-items: center;
    justify-content: center;
    background: #1e1e1e;
    color: #e5e5e5;
    padding: 2rem;
  }

  .welcome-content {
    max-width: 700px;
    text-align: center;
  }

  h1 {
    font-size: 2.2rem;
    margin: 0 0 0.5rem 0;
    font-weight: 600;
    color: #ffffff;
  }

  p {
    font-size: 1rem;
    margin: 0 0 2.5rem 0;
    opacity: 0.8;
  }

  .features {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(170px, 1fr));
    gap: 1.25rem;
  }

  .feature-card {
    background: #252526;
    border: 1px solid #333;
    border-radius: 8px;
    padding: 1.25rem;
    transition: transform 0.15s, border-color 0.15s;
  }

  .feature-card:hover {
    transform: translateY(-3px);
    border-color: #007acc;
  }

  .icon {
    width: 32px;
    height: 32px;
    margin-bottom: 0.75rem;
  }

  .feature-card h3 {
    font-size: 0.95rem;
    margin: 0 0 0.35rem 0;
    font-weight: 600;
    color: #ffffff;
  }

  .feature-card p {
    font-size: 0.8rem;
    margin: 0;
    opacity: 0.75;
    line-height: 1.3;
  }
</style>
