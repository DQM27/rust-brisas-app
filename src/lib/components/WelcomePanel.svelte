

<script lang="ts">
  // Panel de bienvenida o dashboard
</script>

<div class="welcome-panel">
  <div class="welcome-content">
    <h1>Bienvenido a Brisas App</h1>
    <p>Sistema de gestión de usuarios</p>
    
    <div class="features">
      <div class="feature-card">
        <div class="feature-icon">👤</div>
        <h3>Gestión de Usuarios</h3>
        <p>Crea, edita y administra usuarios del sistema</p>
      </div>
      
      <div class="feature-card">
        <div class="feature-icon">🔐</div>
        <h3>Autenticación Segura</h3>
        <p>Sistema de login con encriptación Argon2</p>
      </div>
      
      <div class="feature-card">
        <div class="feature-icon">📊</div>
        <h3>Panel de Control</h3>
        <p>Visualiza estadísticas y métricas en tiempo real</p>
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
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    color: white;
    padding: 2rem;
  }

  .welcome-content {
    max-width: 600px;
    text-align: center;
  }

  h1 {
    font-size: 2.5rem;
    margin: 0 0 0.5rem 0;
    font-weight: 700;
  }

  p {
    font-size: 1.125rem;
    margin: 0 0 3rem 0;
    opacity: 0.9;
  }

  .features {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(150px, 1fr));
    gap: 1.5rem;
  }

  .feature-card {
    background: rgba(255, 255, 255, 0.1);
    backdrop-filter: blur(10px);
    border-radius: 12px;
    padding: 1.5rem;
    transition: transform 0.2s, background 0.2s;
  }

  .feature-card:hover {
    transform: translateY(-4px);
    background: rgba(255, 255, 255, 0.15);
  }

  .feature-icon {
    font-size: 2.5rem;
    margin-bottom: 1rem;
  }

  .feature-card h3 {
    font-size: 1rem;
    margin: 0 0 0.5rem 0;
    font-weight: 600;
  }

  .feature-card p {
    font-size: 0.875rem;
    margin: 0;
    opacity: 0.8;
    line-height: 1.4;
  }
</style>