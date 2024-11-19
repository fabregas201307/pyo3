FROM rust:1.82.0
RUN apt-get update \
    && apt-get install -y vim \
    && apt-get install -y python3.11-venv

# Install the nightly Rust toolchain
RUN rustup install nightly \
    && rustup default nightly

COPY . /opt/pyo3

RUN cd /opt/pyo3/pyo3 && cargo update
RUN python3 -m venv /opt/venv \
    && chmod -R 755 /opt/venv
RUN . /opt/venv/bin/activate && pip install -r /opt/pyo3/requirements.txt

# Set the default Python executable to the virtual environment
ENV PATH="/opt/venv/bin:$PATH"

# Default work directory
WORKDIR /opt

# Default command
CMD ["bash"]