apt-get install -y curl
curl -SLO https://deb.nodesource.com/nsolid_setup_deb.sh
chmod 500 nsolid_setup_deb.sh
./nsolid_setup_deb.sh 21
apt-get install nodejs -y
mkdir "/.npm"
chown -R 1001:123 "/.npm"
npm install -g @angular/cli