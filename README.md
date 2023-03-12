# DevOps Dreamland

This is a playground to implement and try out different DevOps
techniques and tools. It serves my personal learning and as a proof of
concept.

This repository contains a demo application to simulate the software
development lifecycle of a real application. It is deployed to a
Kubernetes cluster. The deployment configuration is stored in the
[devops-dreamland-config](https://github.com/tfkhim/devops-dreamland-config)
repository.

**Disclaimer:** This is mostly a proof of concept. Many implementation
details may not be as elegant as they could be. The focus is on the
interactions between the various parts and not the best possible
implementation of each part.

# Goals

The following section describes some of the goals this project tries
to achieve.

## Automation

Manual repetitive work is boring and error-prone. Therefore it is
essential to automate tasks that have to be repeated for each commit
or deployment. Good examples for such tasks are tests, linting,
dependency version increments, creating release information, release
packaging and deployment. Last but not least automation is a key
requirement for some of the other goals.

## Reproducibility

For me reproducibility means the following: A deployed application
should be a function (in the mathematical sense) of all of its
inputs. This is a very strict definition and it might not be
necessary to be so strict about it. You could embed a build timestamp
into your application that leads to a different result each time you
build your application. For most of us this will be still fine because
the overall behavior of the application doesn't change.

Reproducibility reduces the complexity of finding the root cause for
some unwanted behavior. Imagin each build could have different behavior
even if the inputs (e.g. commits) didn't change. In such a scenario you
would never know if a change in the source code caused some change in
behavior or something completely different.

An example for bad reproducibility are dependencies version ranges.
Every dependency resolution could lead to different version values.

Reproducibility is also only possible if all steps are automated.
Manual steps can always introduce changes in the output due to
accidental mistakes.

## Auditability

## Fun To Work With

# Release Process

## High Level View

## Details

### Continuous Integration

### Continuous Deployment
