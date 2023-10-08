FROM ghcr.io/charmbracelet/vhs
RUN apt-get update && apt-get install -y curl sudo
ARG USERNAME=vhs-user
ARG USER_UID=1000
ARG USER_GID=$USER_UID

RUN groupadd --gid $USER_GID $USERNAME \
    && useradd --uid $USER_UID --gid $USER_GID -m $USERNAME \
    && echo $USERNAME ALL=\(root\) NOPASSWD:ALL > /etc/sudoers.d/$USERNAME \
    && chmod 0440 /etc/sudoers.d/$USERNAME

# This could record on release instead of local build:
# RUN curl -s https://raw.githubusercontent.com/strowk/tisq/main/install.sh | bash

COPY ./tisq /usr/local/bin/tisq
RUN chmod a+x /usr/local/bin/tisq

RUN apt-get install -y git
USER $USERNAME