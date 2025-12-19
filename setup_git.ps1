# Configuración PRO de Git para Windows
# Autor: Antigravity & User

# 1. Configuración de Idioma ( Español )
$env:LANG = 'es_ES'
[Environment]::SetEnvironmentVariable("LANG", "es_ES", "User")
Write-Host "✅ Idioma configurado a Español (persistente)." -ForegroundColor Green

# 2. Identidad del Usuario
$currentName = git config --global user.name
$currentEmail = git config --global user.email

if (-not $currentName) {
    $name = Read-Host "Ingresa tu Nombre para Git"
    git config --global user.name "$name"
}
if (-not $currentEmail) {
    $email = Read-Host "Ingresa tu Email para Git"
    git config --global user.email "$email"
}

# 3. Configuración del Núcleo
git config --global core.editor "code --wait"
git config --global init.defaultBranch main
git config --global core.autocrlf true
git config --global pull.rebase true
git config --global fetch.prune true
git config --global rerere.enabled true

# 4. Alias PRO
git config --global alias.st status
git config --global alias.co checkout
git config --global alias.ci commit
git config --global alias.br branch
git config --global alias.undo "reset HEAD~1 --mixed"
git config --global alias.amend "commit --amend"
git config --global alias.lg "log --graph --abbrev-commit --decorate --format=format:'%C(bold blue)%h%C(reset) - %C(bold green)(%ar)%C(reset) %C(white)%s%C(reset) %C(dim white)- %an%C(reset)%C(bold yellow)%d%C(reset)' --all"

Write-Host "✅ Configuraciones Globales y Alias PRO aplicados." -ForegroundColor Green

# 5. Generación de Archivos Estándar (Si no existen)

$gitAttributesContent = @'
* text=auto eol=lf

*.ts text eol=lf
*.js text eol=lf
*.jsx text eol=lf
*.tsx text eol=lf
*.svelte text eol=lf
*.css text eol=lf
*.html text eol=lf
*.json text eol=lf
*.md text eol=lf
*.yml text eol=lf
*.yaml text eol=lf

*.rs text eol=lf
*.toml text eol=lf
Cargo.lock text eol=lf

*.sh text eol=lf

.gitignore text eol=lf
.gitattributes text eol=lf
.cursorrules text eol=lf
*.config.* text eol=lf
package.json text eol=lf
package-lock.json text eol=lf

*.bat text eol=crlf
*.cmd text eol=crlf
*.ps1 text eol=crlf

*.png binary
*.jpg binary
*.jpeg binary
*.gif binary
*.ico binary
*.svg text eol=lf
*.xml text eol=lf
*.woff binary
*.woff2 binary
*.ttf binary
'@

$editorConfigContent = @'
root = true

[*]
charset = utf-8
end_of_line = lf
insert_final_newline = true
trim_trailing_whitespace = true

[*.{ts,js,svelte,css,html,json}]
indent_style = space
indent_size = 2

[*.rs]
indent_style = space
indent_size = 4

[*.md]
trim_trailing_whitespace = false
'@

if (-not (Test-Path ".gitattributes")) {
    Set-Content -Path ".gitattributes" -Value $gitAttributesContent -Encoding UTF8
    Write-Host "✅ Archivo .gitattributes creado." -ForegroundColor Cyan
} else {
    Write-Host "ℹ️ .gitattributes ya existe, no se modificó." -ForegroundColor Yellow
}

if (-not (Test-Path ".editorconfig")) {
    Set-Content -Path ".editorconfig" -Value $editorConfigContent -Encoding UTF8
    Write-Host "✅ Archivo .editorconfig creado." -ForegroundColor Cyan
} else {
    Write-Host "ℹ️ .editorconfig ya existe, no se modificó." -ForegroundColor Yellow
}

Write-Host "`n🚀 ¡Git configurado nivel PRO! Listo para trabajar." -ForegroundColor Green
