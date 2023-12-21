FROM rust:latest
RUN apt-get update \
    && apt-get install -y vim \
    && apt-get install -y python3.11-venv

RUN cd /opt \
    &&  git clone https://github.com/fabregas201307/pyo3.git --branch kai
RUN cd /opt/pyo3/pyo3 && cargo update
RUN python3 -m venv /opt/venv
RUN . /opt/venv/bin/activate && pip install -r /opt/pyo3/requirements.txt
