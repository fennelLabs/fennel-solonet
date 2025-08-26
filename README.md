Fennel Node Helm Repository

This is the Helm repository for Fennel blockchain node charts.
Usage

# Add the repository
helm repo add fennel https://corruptedaesthetic.github.io/fennel-solonet

# Update repositories
helm repo update

# Install Fennel node
helm install fennel-node fennel/fennel-node

Available Charts

    fennel-node - Fennel blockchain node deployment chart
