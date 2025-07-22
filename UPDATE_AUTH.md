# Authentication Update Guide

This guide helps you update your existing Rumiland CRM deployment to include the new authentication system.

## Steps to Update

### 1. Build the Updated Version

On your local machine:

```bash
# Pull latest changes
git pull

# Build release version
cargo build --release
```

### 2. Backup Your Database

On your server:

```bash
# Create backup
cd /opt/rumiland
cp rumiland.db rumiland_backup_$(date +%Y%m%d).db
```

### 3. Deploy the Update

Use the deployment script or manually:

```bash
# Using deployment script
./deploy.sh

# Or manually:
# Upload new files to server
# Stop service
sudo systemctl stop rumiland

# Replace binary and templates
# ... (follow previous deployment steps)

# Start service
sudo systemctl start rumiland
```

### 4. Apply Database Migration (if needed)

If the automatic migration doesn't work, apply manually:

```bash
cd /opt/rumiland
sqlite3 rumiland.db < migrate_auth.sql
```

### 5. Test the Authentication

1. Visit your CRM URL
2. You should be redirected to login page
3. Login with default credentials:
   - Username: `admin`
   - Password: `admin123`

### 6. Important Security Steps

After successful login:

1. **Create a new admin account** with a strong password
2. **Delete or change the default admin password**
3. **Create accounts for your team members**

## New Features

- 🔐 **Secure Login**: All users must authenticate
- 👥 **User Management**: Admins can create/delete users
- 🛡️ **Role-Based Access**: Admin and regular user roles
- 🚪 **Session Management**: 24-hour sessions with secure cookies
- 📊 **User Info Display**: Shows current user in navigation

## Troubleshooting

### Can't login?
- Check if database migration was applied
- Verify the service is running: `sudo systemctl status rumiland`
- Check logs: `sudo journalctl -u rumiland -n 50`

### Database errors?
- Restore from backup: `cp rumiland_backup_[date].db rumiland.db`
- Re-run the migration script

### Session issues?
- Clear browser cookies
- Restart the service
- Check that your server time is correct

## Security Best Practices

1. **Use strong passwords** (minimum 12 characters)
2. **Regularly rotate passwords**
3. **Monitor user access logs**
4. **Keep the system updated**
5. **Use HTTPS in production** (update Nginx config)

## Need Help?

If you encounter issues:
1. Check the service logs
2. Verify database integrity
3. Ensure all files have correct permissions
4. Contact support with error messages