# Rust API client for openapi

Manage Entrywan resources programmatically using the API.

All API requests are authenticated using [IAM
tokens](https://entrywan.com/docs#iam).  Tokens can be generated
and retrieved from the [portal](https://portal.entrywan.com).  The
portal itself is an API client that uses an unrestricted token to
access resources for an account.

This documentation is generated using an OpenAPI 3.1.0
[specification](https://spec.openapis.org/oas/latest.html).  More
information about OpenAPI can be found on its
[site](https://openapis.org).  The current version of [Entrywan's
API spec](https://entrywan.com/openapi.yaml) is also available for
inspection.

On the left of this page are links to the <i>Endpoints</i> grouped
by tag and <i>Schemas</i> the API exposes.  <i>Endpoints</i> are
URLs that can be accessed with any HTTP client or device.
<i>Schemas</i> are machine-readable data models that represent
resources.

To learn more, have a look at the
[documentation](https://entrywan.com/docs).  If you have any
questions, contact [support](mailto:support@entrywan.com) or your
account representative.


## Overview

This API client was generated by the [OpenAPI Generator](https://openapi-generator.tech) project.  By using the [openapi-spec](https://openapis.org) from a remote server, you can easily generate an API client.

- API version: v1beta
- Package version: v1beta
- Build package: `org.openapitools.codegen.languages.RustClientCodegen`

## Installation

Put the package under your project folder in a directory named `openapi` and add the following to `Cargo.toml` under `[dependencies]`:

```
openapi = { path = "./openapi" }
```

## Documentation for API Endpoints

All URIs are relative to *https://api.entrywan.com/v1beta*

Class | Method | HTTP request | Description
------------ | ------------- | ------------- | -------------
*ClusterApi* | [**cluster_get**](docs/ClusterApi.md#cluster_get) | **GET** /cluster | List kubernetes clusters
*ClusterApi* | [**cluster_id_delete**](docs/ClusterApi.md#cluster_id_delete) | **DELETE** /cluster/{id} | Delete cluster
*ClusterApi* | [**cluster_id_get**](docs/ClusterApi.md#cluster_id_get) | **GET** /cluster/{id} | Get a kubernetes cluster
*ClusterApi* | [**cluster_id_kubeconfig_get**](docs/ClusterApi.md#cluster_id_kubeconfig_get) | **GET** /cluster/{id}/kubeconfig | Fetch the kubeconfig file for a cluster
*ClusterApi* | [**cluster_id_rotate_patch**](docs/ClusterApi.md#cluster_id_rotate_patch) | **PATCH** /cluster/{id}/rotate | Rotate credentials
*ClusterApi* | [**cluster_id_scale_put**](docs/ClusterApi.md#cluster_id_scale_put) | **PUT** /cluster/{id}/scale | Scale a cluster up or down
*ClusterApi* | [**cluster_post**](docs/ClusterApi.md#cluster_post) | **POST** /cluster | Create a kubernetes cluster
*FirewallApi* | [**firewall_get**](docs/FirewallApi.md#firewall_get) | **GET** /firewall | List firewalls
*FirewallApi* | [**firewall_id_delete**](docs/FirewallApi.md#firewall_id_delete) | **DELETE** /firewall/{id} | Delete a firewall
*FirewallApi* | [**firewall_post**](docs/FirewallApi.md#firewall_post) | **POST** /firewall | Add a firewall
*FirewallApi* | [**instance_id_firewall_put**](docs/FirewallApi.md#instance_id_firewall_put) | **PUT** /instance/{id}/firewall | Apply firewall to instance
*InstanceApi* | [**instance_get**](docs/InstanceApi.md#instance_get) | **GET** /instance | List instances
*InstanceApi* | [**instance_id_delete**](docs/InstanceApi.md#instance_id_delete) | **DELETE** /instance/{id} | Delete an instance
*InstanceApi* | [**instance_id_get**](docs/InstanceApi.md#instance_id_get) | **GET** /instance/{id} | Get instance by ID
*InstanceApi* | [**instance_id_reboot_patch**](docs/InstanceApi.md#instance_id_reboot_patch) | **PATCH** /instance/{id}/reboot | Reboot an instance
*InstanceApi* | [**instance_id_snapshot_get**](docs/InstanceApi.md#instance_id_snapshot_get) | **GET** /instance/{id}/snapshot | List instance snapshots
*InstanceApi* | [**instance_id_snapshot_name_delete**](docs/InstanceApi.md#instance_id_snapshot_name_delete) | **DELETE** /instance/{id}/snapshot/{name} | Delete a snapshot
*InstanceApi* | [**instance_id_snapshot_name_put**](docs/InstanceApi.md#instance_id_snapshot_name_put) | **PUT** /instance/{id}/snapshot/{name} | Revert to snapshot
*InstanceApi* | [**instance_id_snapshot_put**](docs/InstanceApi.md#instance_id_snapshot_put) | **PUT** /instance/{id}/snapshot | Snapshot an instance
*InstanceApi* | [**instance_id_start_patch**](docs/InstanceApi.md#instance_id_start_patch) | **PATCH** /instance/{id}/start | Start an instance
*InstanceApi* | [**instance_id_stop_patch**](docs/InstanceApi.md#instance_id_stop_patch) | **PATCH** /instance/{id}/stop | Stop an instance
*InstanceApi* | [**instance_post**](docs/InstanceApi.md#instance_post) | **POST** /instance | Add a new instance
*LoadBalancerApi* | [**loadbalancer_get**](docs/LoadBalancerApi.md#loadbalancer_get) | **GET** /loadbalancer | List load balancers
*LoadBalancerApi* | [**loadbalancer_id_delete**](docs/LoadBalancerApi.md#loadbalancer_id_delete) | **DELETE** /loadbalancer/{id} | Delete load balancer
*LoadBalancerApi* | [**loadbalancer_id_put**](docs/LoadBalancerApi.md#loadbalancer_id_put) | **PUT** /loadbalancer/{id} | Update load balancer targets
*LoadBalancerApi* | [**loadbalancer_post**](docs/LoadBalancerApi.md#loadbalancer_post) | **POST** /loadbalancer | Create a load balancer
*SsHkeyApi* | [**sshkey_get**](docs/SsHkeyApi.md#sshkey_get) | **GET** /sshkey | List SSH keys
*SsHkeyApi* | [**sshkey_id_delete**](docs/SsHkeyApi.md#sshkey_id_delete) | **DELETE** /sshkey/{id} | Delete SSH key
*SsHkeyApi* | [**sshkey_post**](docs/SsHkeyApi.md#sshkey_post) | **POST** /sshkey | Create SSH key
*TagApi* | [**tag_id_get**](docs/TagApi.md#tag_id_get) | **GET** /tag/{id} | Get tags for a resource ID
*TagApi* | [**tag_id_patch**](docs/TagApi.md#tag_id_patch) | **PATCH** /tag/{id} | Removes tags for a resource ID
*TagApi* | [**tag_id_put**](docs/TagApi.md#tag_id_put) | **PUT** /tag/{id} | Set tags for a resource ID
*VpcApi* | [**vpc_get**](docs/VpcApi.md#vpc_get) | **GET** /vpc | List VPC
*VpcApi* | [**vpc_id_delete**](docs/VpcApi.md#vpc_id_delete) | **DELETE** /vpc/{id} | Delete VPC
*VpcApi* | [**vpc_id_patch**](docs/VpcApi.md#vpc_id_patch) | **PATCH** /vpc/{id} | Remove member from VPC
*VpcApi* | [**vpc_id_post**](docs/VpcApi.md#vpc_id_post) | **POST** /vpc/{id} | Create a VPC
*VpcApi* | [**vpc_id_put**](docs/VpcApi.md#vpc_id_put) | **PUT** /vpc/{id} | Add member to VPC


## Documentation For Models

 - [Cluster](docs/Cluster.md)
 - [ClusterIdScalePutRequest](docs/ClusterIdScalePutRequest.md)
 - [ClusterPostRequest](docs/ClusterPostRequest.md)
 - [Firewall](docs/Firewall.md)
 - [FirewallPostRequest](docs/FirewallPostRequest.md)
 - [FirewallPostRequestRulesInner](docs/FirewallPostRequestRulesInner.md)
 - [Instance](docs/Instance.md)
 - [InstancePost200Response](docs/InstancePost200Response.md)
 - [InstancePostRequest](docs/InstancePostRequest.md)
 - [LoadBalancer](docs/LoadBalancer.md)
 - [LoadbalancerPostRequest](docs/LoadbalancerPostRequest.md)
 - [LoadbalancerPostRequestListenersInner](docs/LoadbalancerPostRequestListenersInner.md)
 - [LoadbalancerPostRequestListenersInnerTargetsInner](docs/LoadbalancerPostRequestListenersInnerTargetsInner.md)
 - [SsHkey](docs/SsHkey.md)
 - [SshkeyPostRequest](docs/SshkeyPostRequest.md)
 - [Vpc](docs/Vpc.md)
 - [VpcIdPatchRequest](docs/VpcIdPatchRequest.md)
 - [VpcIdPostRequest](docs/VpcIdPostRequest.md)
 - [VpcIdPutRequest](docs/VpcIdPutRequest.md)
 - [VpcMembersInner](docs/VpcMembersInner.md)


To get access to the crate's generated documentation, use:

```
cargo doc --open
```

## Author


